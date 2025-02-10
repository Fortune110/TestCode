fn main() {
    println!("--- clyde-3g-eps-service starting ---");

   
    cubeos_common::hello_common();

    clyde_3g_eps_service::init_service();

    println!("--- clyde-3g-eps-service finished ---");
}