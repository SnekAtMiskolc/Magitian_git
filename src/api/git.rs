use crate::models::git::GitPath;
use crate::models::git::Service;
use std::ascii::AsciiExt;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::process;
use std::process::Stdio;
use std::str;

use actix_web::HttpRequest;
use actix_web::{get, post, web, HttpResponse, Responder};

use encoding::all::ISO_8859_1;
use encoding::{EncoderTrap, Encoding};

#[get("/{u}/{r}/info/refs")]
pub async fn info_refs(
    s: web::Query<Service>,
    p: web::Path<GitPath>,
    _r: HttpRequest,
) -> HttpResponse {
    let s = s.service.clone();
    let rt = format!("application/x-{}-advertisement", s);

    let c = process::Command::new(&s)
        .arg("--advertise-refs")
        .arg("--stateless-rpc")
        .arg(format!("./git/{}/{}.git/", p.u, p.r))
        .output()
        .expect("Cannot execute git command!");

    let refs = String::from_utf8(c.stdout).unwrap_or_default();

    let mut pp = String::new();

    if s == "git-upload-pack" {
        pp = String::from("001e")
    } else if s == "git-receive-pack" {
        pp = String::from("001f");
    }

    HttpResponse::Ok()
        .insert_header(("Content-Type", rt))
        .insert_header(("Cache-Control", "no-cache"))
        .body(format!("{}# service={}\n0000{}", pp, s, refs))
}

#[post("/{u}/{r}/{s}")]
pub async fn service_rpc(
    _r: HttpRequest,
    gr: web::Bytes,
    p: web::Path<(String, String, String)>,
) -> HttpResponse {
    let s = p.2.to_owned();
    let rt = format!("application/x-{}-result", s);

    let cmd = match process::Command::new(s)
        .arg("--stateless-rpc")
        .arg(format!("./git/{}/{}.git/", p.0, p.1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(msg) => match msg.kind() {
            ErrorKind::NotFound => return HttpResponse::NotFound().body("This repo does not exist"),
            _ => return HttpResponse::InternalServerError().body("Oops something went wrong")
        },
        Ok(proc) => proc,
    };

    match cmd
        .stdin
        .unwrap()
        .write_all(&*gr /*web::Bytes derefs into a [u8]*/)
    {
        Err(msg) => panic!("Couldn't write to cmd stdin: {}", msg),
        Ok(_) => println!("wrote to stdin in cmd."),
    }

    let mut rb: Vec<u8> = vec![];
    match cmd.stdout.unwrap().read_to_end(&mut rb) {
        Err(msg) => panic!("Couldn't read stdin to stdout: {}", msg),
        Ok(_) => println!("response body succesfully created",),
    }

    HttpResponse::Ok()
        .insert_header(("Content-Type", rt))
        .body(rb)

    // Now that i am finally done implementing this horribly undocumented protocol it's time
    // to delete all code comments so that others have to suffer the same way i did.
}
