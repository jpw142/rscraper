#![allow(unreachable_code)]
#![allow(dead_code)]
use std::{time::Duration, fs::File};
use thirtyfour::prelude::*;
use tokio::{self, time::sleep, task::{self},};
use std::process::Command;
use thirtyfour::stringmatch::StringMatch;
/*
Note to future jack
please fix your code

Note from future jack
no :)
*/
#[derive(Debug, Clone, PartialEq)]
enum EventType {
    Donation(String, u16),
    Message(String),
    Follow,
    Shared,
    Join,
}

#[derive(Debug, Clone, PartialEq)]
struct Event {
    user: String,
    payload: EventType,
}

/* STARTUP PROCESS:
Open cmd prompt and in type these
chrome.exe --remote-debugging-port=9222 --user-data-dir="C:\Users\jackw\OneDrive\Desktop\Data"
chromedriver
*/

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> WebDriverResult<()> {

    // Launch Chrome Remote debugging Instance
    task::spawn(async {
    let _ = Command::new("chrome.exe")
        .arg("--remote-debugging-port=9222")
        .arg(r#"--user-data-dir=C:\Users\jackw\OneDrive\Desktop\Data"#)
        .output().unwrap();        
    });

    // Launch Chromedriver Instance
    task::spawn(async {
    let _ = Command::new("chromedriver")
        .output().unwrap();        
    });

    
    
    // Launch chromedriver
    //let _ = Command::new("chromedriver")
    //    .output().unwrap();
    sleep(Duration::from_millis(2000)).await;    
    // Load google account info
    let file = File::open("sensitiveinfo.json").expect("JSON ERROR JSON ERROR WEEWOOWEEWOO");
    let json: serde_json::Value = serde_json::from_reader(&file).expect("READER ERROR READER ERROR WEEWOOWEEWOO");
    let email = json["email"].as_str().unwrap();
    let password = json["password"].as_str().unwrap();
    let username = json["account"].as_str().unwrap();

    // Get the desired tik tok live chat link
    let url = format!("https://www.tiktok.com/{}/live",username);
    
    // Attach webdriver to existing google chrome instance to avoid login shenanigans
    let mut caps = DesiredCapabilities::chrome();
    // This localhost is the localhost you run chrome remote debugger on with the command
    caps.set_debugger_address("localhost:9222")?;
    // This localhost is the port chromedriver spits out
    let d = WebDriver::new("http://localhost:9515", caps).await?;
    
    // Go to tik tok live page
    d.goto(url).await?;
    
    // Login
    // Checks if the login screen comes up
    let login = d.find(By::Css(".tiktok-aiuhe9-DivModalContent")).await;
    if let Ok(_) = login {
        // If found we should login
        // Wait for sign in screen and click sign in with email
        let sb = d.query(By::Css("div.tiktok-2pt368-DivBoxContainer:nth-child(5)")).first().await?;
        sb.wait_until().displayed().await?;
        sb.click().await?;
        sleep(Duration::from_millis(2500)).await;
        
        // Get list of windows and switch to login window, empiracally I've found the second window is the login, this may be wrong
        let windows = d.windows().await?;
        d.switch_to_window(windows[1].clone()).await?;
        sleep(Duration::from_millis(1500)).await;

        // If we already have our email saved in then click it
        let prelog = d.find(By::Css(".tgnCOd")).await;
        if let Ok(element) = prelog {
            element.click().await?;
        }
        else {
            // Enter email into email box
            let ee = d.query(By::Css("#identifierId")).first().await?;
            ee.send_keys(email).await?;
            // Go to next page
            let next = d.query(By::Css(".VfPpkd-LgbsSe-OWXEXe-k8QpJ > span:nth-child(4)")).first().await?;
            next.click().await?;
            sleep(Duration::from_millis(2000)).await;
            // Enter password into password box
            let ep = d.query(By::Css("#password > div.aCsJod.oJeWuf > div > div.Xb9hP > input")).first().await?;
            ep.send_keys(password).await?;
            // Go to next page
            let next2 = d.query(By::Css(".VfPpkd-LgbsSe-OWXEXe-k8QpJ > span:nth-child(4)")).first().await?;
            next2.click().await?;
            // Life goes fast, just take a break and let things load for a second
            sleep(Duration::from_millis(5000)).await;
        }
    }
    // By this point the page is loading and has been logged in successfully
    //
    // Search for each elements class name

    // Attatch to all elements that would like to be tracked
    let chatquery = d.query(By::Css("div[class*='DivChatMessageList']")).first().await;
    if chatquery.is_err() {panic!("Chat failed to attach")}
    let chat = chatquery?;
    println!("[-] Chat Attached");

    let donationbarquery = d.query(By::Css("div[class*='StyledGiftTray']")).with_tag("div").first().await;
    if donationbarquery.is_err() {panic!("Donation Bar failed to attach")}
    let donationbar = donationbarquery?;
    println!("[-] DonationBar Attached");

    let stickybarquery = d.query(By::Css("div[class*='DivBottomStickyMessageContainer']")).first().await;
    if stickybarquery.is_err() {panic!("Sticky Bar failed to attatch")}
    let stickybar = stickybarquery?;
    println!("[-] StickyBar Attached");

    let chatbox = d.query(By::Css("div[class*='DivCommentContainer']")).first().await?;
    let messageboxquery = chatbox.query(By::ClassName("tiktok-ahx06z-DivEditor")).first().await;
    if messageboxquery.is_err() {panic!("Message Box failed to attatch")}
    println!("[-] Chatbox Attached");

    // Chatter
    println!("[-] Initializing Chatter");
    let mut last_message = chat.clone();
    let chatter = task::spawn(async move {
        loop{
            let mut chatmessages = chat.clone().find_all(By::Css("div[class*='DivChatMessage']")).await?;
        
            // Delete all old info
            'outer: for (i, message) in chatmessages.clone().iter_mut().enumerate() {
                // If we find that a message in new scan is equal to old scan, delete all elements before that because they're old
                if message == &last_message.clone() {
                    for _ in 0..=i {
                        chatmessages.remove(0);
                    }
                    break 'outer;
                }
            }

            // If there is no new chat messages then why would we print or do anything silly
            if chatmessages.len() == 0 {
                continue;
            }

            last_message = chatmessages[chatmessages.len() -1].clone();
            for message in chatmessages {
                // If you are getting nonsense with messages not showing its' definitly because not handling these errors
                let userinfo = message.find(By::Css("span[class*='SpanEllipsisName']")).await?;
                let comment = message.find(By::Css("div[class*='DivComment']")).await?;
                println!("{}: {}", userinfo.inner_html().await?, comment.inner_html().await?);

            }
        }
        Ok::<(), WebDriverError>(())
    });
    println!("[-] Chatter Initialized");

    // Donater
    println!("[-] Initializing Donater");
    let mut old_donations: Vec<Event> = vec![];
    let mut event_donations: Vec<Event> = vec![];
    let donater = task::spawn(async move {
        loop{
            let donations = donationbar.clone().find_all(By::Css("[class*='DivSendGift']")).await.expect("piip");

            for donation in donations {
                let multiplier = donation.find(By::Css("[class*='SpanBullet']")).await;
                let userinfo = donation.find(By::Css("[class*='DivTitleGift']")).await;
                let donation = donation.find(By::Css("[class*='DivDescriptionGift']")).await;
                if let (Ok(m), Ok(ui), Ok(d)) = (multiplier, userinfo, donation) {
                    let mstring = m.inner_html().await?;
                    let uistring = ui.inner_html().await?;
                    let dstring = d.inner_html().await?;
                    event_donations.push(Event{
                        user: uistring,
                        payload: EventType::Donation(dstring, mstring.parse().unwrap())
                    });
                }
            }

            for old_donation in old_donations.clone() {
                if event_donations.contains(&old_donation) {
                    continue;
                }
                let mut donation: String = String::new();
                let mut multiplier: u16 = 0;
                match old_donation.payload.clone() {
                    EventType::Donation(d, m) => {
                        donation = d;
                        multiplier = m;
                    }
                    _ => {}
                }
                if event_donations.contains(&Event{user: old_donation.user.clone(), payload: EventType::Donation(donation, multiplier + 1) }) {
                    continue;
                }
                println!("");
                println!("{:?}", old_donation.clone());
                println!("");
            }
            
            old_donations = event_donations.clone();
            event_donations.clear();

        }
        Ok::<(), WebDriverError>(())
    });
    println!("[-] Donater Initialized");
    
    println!("[-] Joining Handles");
    // println!("{:?}", chatter.await);
    println!("{:?}", donater.await);
    Ok(())
}

async fn sendmessage(message: &str, chatbox: WebElement, messagebox: WebElement) -> WebDriverResult<()> {
    // Enter text
    messagebox.send_keys(message).await?;
    // Click send
    chatbox.query(By::ClassName("tiktok-1dgtn4b-DivPostButton")).first().await?.click().await?;
    Ok(())
}
