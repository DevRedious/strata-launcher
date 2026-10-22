// Pas de fenêtre de console en plus de l'application, en version publiée
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    strata_launcher_lib::run()
}
