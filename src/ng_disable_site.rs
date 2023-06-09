use crate::utils::{reload_nginx, rm_symlink, test_nginx, walk_folder, FileData, ENABLED};
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, MultiSelect};

async fn get_site_names() -> Result<Vec<FileData>> {
    let mut list: Vec<FileData> = vec![];

    let enabled = walk_folder(ENABLED).await?;

    for (_, file) in enabled {
        if file.is_symlink {
            list.push(file)
        }
    }

    list.sort_by(|a, b| a.file_name.cmp(&b.file_name));

    Ok(list)
}

pub async fn ng_disable_site() -> Result<()> {
    let list: Vec<FileData> = get_site_names().await?;
    if !list.is_empty() {
        let multi_selections: Vec<String> = list.into_iter().map(|x| x.file_name).collect();

        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick site(s)")
            .items(&multi_selections[..])
            .interact()?;

        if !selections.is_empty() {
            for selection in selections {
                let file_to_unlink = multi_selections[selection].clone();

                if let Err(err) = rm_symlink(file_to_unlink).await {
                    log::error!("Failed to symlink...");
                    return Err(err);
                }
            }

            test_nginx()?;
            reload_nginx()?;
        }
    } else {
        log::info!("All sites are disabled...");
    }

    Ok(())
}
