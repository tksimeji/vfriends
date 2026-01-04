use tauri::AppHandle;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
use windows::{
    core::{Interface, Result, HSTRING},
    Data::Xml::Dom::{XmlDocument, XmlElement},
    UI::Notifications::{ToastNotification, ToastNotificationManager, ToastTemplateType},
};

pub fn notify(app: &AppHandle, title: &str, body: &str, icon_src: Option<String>) -> Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    }

    let xml = ToastNotificationManager::GetTemplateContent(ToastTemplateType::ToastImageAndText02)?;

    set_text(&xml, title, body)?;
    if let Some(icon_src) = icon_src {
        set_circle_circle(&xml, &icon_src)?;
    }

    let app_id = app.config().identifier.clone();

    let toast = ToastNotification::CreateToastNotification(&xml)?;
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&app_id.into())?;

    notifier.Show(&toast)?;

    Ok(())
}

fn set_text(xml: &XmlDocument, title: &str, body: &str) -> Result<()> {
    let node_list = xml.GetElementsByTagName(&HSTRING::from("text"))?;
    node_list.Item(0)?.SetInnerText(&title.into())?;
    node_list.Item(1)?.SetInnerText(&body.into())?;
    Ok(())
}

fn set_circle_circle(xml: &XmlDocument, src: &str) -> Result<()> {
    let node_list = xml.GetElementsByTagName(&HSTRING::from("image"))?;

    let node = node_list.Item(0)?;

    let element = node.cast::<XmlElement>()?;
    element.SetAttribute(&HSTRING::from("src"), &src.into())?;
    element.SetAttribute(&HSTRING::from("hint-crop"), &HSTRING::from("circle"))?;
    element.SetAttribute(&HSTRING::from("alt"), &HSTRING::from("Profile Icon"))?;

    Ok(())
}
