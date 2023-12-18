use crate::client::{BillWithFiles, Client, DntcFile, DtlVo};
use crate::utils::{config, date};
use async_trait::async_trait;
use bytes::Bytes;
use chrono::prelude::Utc;
use console::Emoji;
use dirs::home_dir;
use git2::{
    self, Commit, Cred, IndexAddOption, ObjectType, Oid, RemoteCallbacks, Repository, Signature,
};
use regex::Regex;
use std::error::Error;
use std::fs::{create_dir, remove_dir_all, File};
use std::io;
use std::path::Path;

static DOCUMENT: Emoji<'_, '_> = Emoji("📑  ", "");

pub struct FileManager<'a> {
    _remote_url: String,
    _local_path: String,
    _local_repo: Option<Repository>,
    _git_signature: Signature<'a>,
}

impl<'a> FileManager<'a> {
    pub async fn new() -> Result<FileManager<'a>, Box<dyn Error>> {
        let _config = config::Config::load_or_new();

        match _config {
            Ok(config) => match config.remote_file_repository {
                Some(remote_file_repository) => {
                    let _remote_url = format!("git@github.com:{}", remote_file_repository);

                    let global_config = git2::Config::open_default().unwrap();

                    let mut fm = FileManager {
                        _local_path: config.local_file_repository.unwrap(),
                        _remote_url: _remote_url,
                        _local_repo: None,
                        _git_signature: Signature::now(
                            &global_config.get_string("user.name").unwrap(),
                            &global_config.get_string("user.email").unwrap(),
                        )
                        .unwrap(),
                    };

                    if !Path::new(&fm._local_path).exists() {
                        fm.clone_remote_repo();
                    }

                    return Ok(fm);
                }
                None => {
                    eprintln!("원격 저장소를 설정해주세요.");
                    panic!();
                }
            },
            Err(_) => {
                eprintln!("ogk 기본 설정을 먼저 진행해주세요.");
                panic!();
            }
        }
    }

    pub fn clone_remote_repo(&mut self) -> &Option<Repository> {
        let _ = remove_dir_all(&self._local_path);

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                // TODO: ssh key 관리 고려
                std::path::Path::new(&format!(
                    "{}/.ssh/id_ed25519",
                    home_dir().unwrap().to_str().unwrap()
                )),
                None,
            )
        });

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        match builder.clone(&self._remote_url, Path::new(&self._local_path)) {
            Ok(repo) => {
                self._local_repo = Some(repo);
            }
            Err(error) => {
                println!("{}", &self._remote_url);
                println!("{}", &self._local_path);
                panic!("{}", error);
            }
        }

        &self._local_repo
    }

    pub async fn download(
        &self,
        client: &Client,
        bill: &BillWithFiles,
        bill_from_list: &DtlVo,
    ) -> Result<Option<Vec<DntcFile>>, Box<dyn std::error::Error>> {
        let config = config::Config::load_or_new()?;
        match config.remote_file_repository {
            Some(_) => {
                let mut downloaded_files: Vec<DntcFile> = vec![];
                let fm = FileManager::new().await.unwrap();

                if let Some(ref file_list) = bill.atchFileList {
                    for file in &*file_list {
                        if fm.has_downloaded(bill, bill_from_list, &file.uploadFileOrginlNm)
                            == false
                        {
                            let downloaded = client.download_file(file).await?;
                            let _ = fm.save(
                                &downloaded,
                                bill,
                                bill_from_list,
                                &file.uploadFileOrginlNm,
                            );
                            downloaded_files.push(file.clone());
                        }
                    }

                    Ok(Some(downloaded_files))
                } else {
                    Ok(Some(downloaded_files))
                }
            }
            None => {
                eprintln!("청구파일을 다운로드 하려면 원격저장소 주소를 먼저 설정해주세요.");
                Ok(None)
            }
        }
    }

    // {접수일자}_{청구_제묵}
    pub fn make_dirname(request_date: &str, request_subject: &str) -> String {
        let re_illegal_symbols = Regex::new("[.\"\n \t()\'~\u{1c}]").unwrap();
        let re_retouch = Regex::new("_+").unwrap();
        format!(
            "{}_{}",
            request_date.replace(".", "-"),
            re_retouch
                .replace_all(
                    &re_illegal_symbols.replace_all(request_subject.trim(), "_"),
                    "_",
                )
                .to_string()
        )
    }

    // {접수번호}_{처리기관이름}_{업로드_파일명}
    pub fn make_filename(
        registration_number: &str,
        rqest_full_instt_name: &str,
        file_name: &str,
    ) -> String {
        let re_illegal_symbols = Regex::new("[\"\n \t()\'?~\u{1c}]").unwrap();
        let re_retouch = Regex::new("_+").unwrap();

        format!(
            "{}_{}_{}",
            registration_number,
            rqest_full_instt_name.replace(" ", "_"),
            re_retouch
                .replace_all(&re_illegal_symbols.replace_all(file_name.trim(), "_"), "_",)
                .to_string()
        )
    }

    pub fn save(
        &self,
        downloaded_file: &Bytes,
        downloadable_bill: &BillWithFiles,
        bill_from_list: &DtlVo,
        orig_file_name: &str,
    ) -> Result<File, Box<dyn std::error::Error>> {
        let dir_path = format!("{}/{}", &self._local_path, downloadable_bill.get_dirname(),);
        let file_path = format!(
            "{}/{}",
            &dir_path,
            downloadable_bill.get_filename(&bill_from_list.prcsFullInsttNm, orig_file_name)
        );

        create_dir(Path::new(&dir_path)).unwrap_or_default();
        let mut local_file = File::create(&file_path)?;
        io::copy(&mut downloaded_file.as_ref(), &mut local_file)?;
        Ok(local_file)
    }

    fn has_downloaded<T: Downloadable>(
        &self,
        downloadable_bill: &T,
        bill_from_list: &DtlVo,
        orig_file_name: &str,
    ) -> bool {
        let dir_path = format!("{}/{}", &self._local_path, downloadable_bill.get_dirname(),);
        let file_path = format!(
            "{}/{}",
            &dir_path,
            downloadable_bill.get_filename(&bill_from_list.prcsFullInsttNm, orig_file_name)
        );

        return Path::new(&file_path).exists();
    }

    fn remote_callbaks(&self) -> RemoteCallbacks<'a> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                std::path::Path::new(&format!(
                    "{}/.ssh/id_ed25519",
                    home_dir().unwrap().to_str().unwrap()
                )),
                None,
            )
        });
        return callbacks;
    }

    pub async fn sync_with_remote(&self) -> Result<(), Box<dyn Error>> {
        let repo = match Repository::open(&self._local_path) {
            Ok(repo) => repo,
            Err(e) => panic!("파일 저장소를 불러오는데 실패하였습니다.: {}", e),
        };

        let callbacks = self.remote_callbaks();
        let mut po = git2::FetchOptions::new();
        let mut po = po.remote_callbacks(callbacks);

        repo.find_remote("origin")?
            .fetch(&["main"], Some(&mut po), None);
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_up_to_date() {
            Ok(())
        } else {
            let refname = format!("refs/heads/main");
            let mut reference = repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()));

            Ok(())
        }
    }

    pub async fn upload(&self) -> Result<Oid, git2::Error> {
        let callbacks = self.remote_callbaks();
        fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
            let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
            obj.into_commit()
                .map_err(|_| git2::Error::from_str("Couldn't find commit"))
        }

        let repo = match Repository::open(&self._local_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        let mut index = repo.index().unwrap();
        let _ = index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None);
        let _ = index.write();
        let oid = index.write_tree().unwrap();
        let parent_commit = find_last_commit(&repo).unwrap();
        let tree = repo.find_tree(oid).unwrap();

        repo.commit(
            Some("HEAD"),
            &self._git_signature,
            &self._git_signature,
            &format!(
                "{} - {}",
                DOCUMENT,
                date::KstDateTime::from(Utc::now()).format(Some("%F %T"))
            ),
            &tree,
            &[&parent_commit],
        )
        .unwrap();

        let mut remote = match repo.find_remote("origin") {
            Ok(r) => r,
            Err(_) => repo.remote("origin", &self._remote_url)?,
        };

        let mut po = git2::PushOptions::new();
        let mut po = po.remote_callbacks(callbacks);

        match remote.push(
            &["refs/heads/main", "refs/remotes/origin/main"],
            Some(&mut po),
        ) {
            Ok(_) => Ok(oid),
            Err(e) => {
                eprintln!("{}", e);
                Ok(oid)
            }
        }
    }
}

#[async_trait]
pub trait Downloadable {
    fn get_filename(&self, prcs_full_instt_nm: &str, orig_file_name: &str) -> String;
    fn get_dirname(&self) -> String;
}

#[cfg(test)]
mod tests {
    use crate::files::FileManager;

    #[tokio::test]
    async fn test_sync_with_remote() {
        let fm = FileManager::new().await.unwrap();
        fm.sync_with_remote().await;
    }
}
