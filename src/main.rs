extern crate reqwest;
use core::panic;
use std::{fs, fmt::{format}, time::Duration, thread, io::Write};

use reqwest::{Response, blocking::Client};
use tokio::time::sleep;
const proxyscrape_api: &str = "https://api.proxyscrape.com/v2/?request=displayproxies&protocol=https&timeout=10000&country=all&ssl=all&anonymity=all";
const httpbin_api: &str = "https://httpbin.org/ip";
static mut success: u32 = 0;
static mut success_list: Vec<String> = Vec::new();

fn open_file_return_list() -> Vec<String>{
    let mut body = reqwest::blocking::get(proxyscrape_api).unwrap().text().unwrap();
    let mut data: &str = &*body;
    let mut list: Vec<&str> = data.split("\r\n").collect();
    let mut finalList: Vec<String> = list.iter().map(|&s|s.into()).collect(); finalList.pop();
    return finalList;
}

fn try_request(address: &str){
    let uri = String::from(format!("{address}"));
    let proxy = reqwest::Proxy::https(uri).unwrap();
    let client = reqwest::blocking::Client::builder().proxy(proxy).build().unwrap();
    let request = client.get(httpbin_api).timeout(Duration::from_secs(10)).send();
    let mut request = match request {
        Ok(file) => file,
        Err(e) => return (unsafe {success = success + 1})
    };
    unsafe {success = success + 1};
    if request.status() == 200{
        println!("Success: {}", address);
        unsafe {success_list.push(address.to_string())};
    }
}

unsafe fn write_addresses(addresses: &Vec<String>){
    let fp = fs::File::create("proxies.txt");
    let mut fp = match fp {
        Ok(f) => f,
        Err(e) => panic!("Error! {:?}", e)
    };
    for i in addresses{
        write!(fp, "{}\n", i);
    }
    println!("Writed {:?} proxies.", addresses.len());
}

fn main(){
    let proxies = open_file_return_list();
    let mut i = proxies.len();
    println!("Proxies loaded: {:?}", i);
    for i in proxies{
        let thread = thread::spawn(move || try_request(&i));
    }
    unsafe{
        while (i - 100) > success.try_into().unwrap(){

        }
        write_addresses(&success_list);
    }
}