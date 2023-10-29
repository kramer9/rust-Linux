
/*
In Rust, all files and folders are modules. In order to use the code in a module, you need to first import it with the mod syntax. 
Essentially this is inserting the code from the module at the point where the mod my_struct; statement is found. 
*/

// https://medium.com/codex/rust-modules-and-project-structure-832404a33e2e#id_token=eyJhbGciOiJSUzI1NiIsImtpZCI6ImEwNmFmMGI2OGEyMTE5ZDY5MmNhYzRhYmY0MTVmZjM3ODgxMzZmNjUiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMTYyOTYwMzU4MzQtazFrNnFlMDYwczJ0cDJhMmphbTRsamRjbXMwMHN0dGcuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMTYyOTYwMzU4MzQtazFrNnFlMDYwczJ0cDJhMmphbTRsamRjbXMwMHN0dGcuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTUxNDk1MDU2ODY1MzYzNTI1MjQiLCJlbWFpbCI6ImtyYW1lcjkyMDAyQGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJuYmYiOjE2OTg0MzU3MjYsIm5hbWUiOiJEIEtyYW1lciIsInBpY3R1cmUiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BQ2c4b2NMbmIxd2F6UjhNWFhZR2txczBSRjhxTTBYbWFKYWpUUmlabzFnTE9kNGk9czk2LWMiLCJnaXZlbl9uYW1lIjoiRCIsImZhbWlseV9uYW1lIjoiS3JhbWVyIiwibG9jYWxlIjoiZW4iLCJpYXQiOjE2OTg0MzYwMjYsImV4cCI6MTY5ODQzOTYyNiwianRpIjoiZjU5ZDI1YTc3MzJhMzI2MTliOWExYTFjN2M0ODA4ZjE3MzFmNDJmMSJ9.awP_gfsrikySVQHCeBEUr0UvjjUyXkqsGsi9zWnMQIaASoC3v-muNKdcyn6TtLi5yxHibxpNsWi1NcsdUyf3JaB8ljHfauquzngpQbdpQRTb6eG_wh0APBoCgshFgmRtrimRYmLtzFpjZyWv-k73RI_kjsEKERXFzhggt0CuBYrXGpKugI4_RUTF4R2AwgqWrKWJFLX-JBw7VWzDPW7LQlZpN2sH4Z_oDzaLdgUYWiCzI1tdwMYHZqvJofL3IoBZ7A4hRsslXPKSpYfhcfV3nFxuCpO-YHMaSzwHoZnQJ5hobHwdbgDTriWo9lqluTAs9sZKcfAWQCNaVIncTdNWsw

// \|/ so the mod scripts inserts the all of the functions listed in scripts.rs into the existing crate
mod common;

fn main() {
    common::fromcommon::sayhello();
    // basically the full path to the function
    // scripts is the directory
    // func is the source code file name
    // sayhello is the published function name
    // but rust talk has them called -  Paths are comprised of crates, modules, and items.
}