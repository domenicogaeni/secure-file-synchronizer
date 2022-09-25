use self::models::*;
use diesel::prelude::*;
use secure_file_synchronizer::*;

fn main() {
    use self::schema::uploaded_files::dsl::*;

    let connection = &mut establish_connection();
    let results = uploaded_files
        .limit(5)
        .load::<UploadedFile>(connection)
        .expect("Error loading posts");

    println!("Displaying {} uploaded files", results.len());
    for file in results {
        println!("{}", file.path);
        println!("-----------\n");
        println!("{}", file.hash);
    }
}
