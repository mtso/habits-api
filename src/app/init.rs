use habits::{externals, processors, resources};
use rocket;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocksdb::DB;
use std::sync::{Arc, Mutex};

pub fn init_app() -> Result<Rocket, String> {
    let app = rocket::ignite();

    let app = app.attach(AdHoc::on_attach(manage_dbconn));

    // Set up routes.
    let app = app.mount(
        "/api/habits",
        routes![
            resources::healthcheck,
            // Habits
            resources::habits_resource::post_habit,
            resources::habits_resource::get_habit,
            resources::habits_resource::delete_habit,
            resources::habits_resource::check_habit,
            resources::habits_resource::uncheck_habit,
            resources::habits_resource::reset_habit_checks,
        ],
    );

    let app = app.catch(catchers![
        resources::catchers::not_found,
        resources::catchers::bad_request,
        resources::catchers::internal_server_error,
    ]);

    Ok(app)
}

fn manage_dbconn(app: Rocket) -> Result<Rocket, Rocket> {
    debug!("Adding dbconn managed state from config...");

    let dbpath = app
        .config()
        .get_str("rocksdb_path")
        .expect("Configure rocksdb_path in Rocket.toml")
        .to_string();

    let rocksdb = must_init_db(dbpath);

    // Share the db connection.
    let dbconn = Arc::new(Mutex::new(rocksdb));

    // Set up processors.
    let habits_storage = externals::HabitsStorageRocksdb::new(dbconn.clone());
    let habits_processor = processors::HabitsProcessor::new(Box::new(habits_storage));

    // Set up managed state.
    let app = app.manage(habits_processor);

    Ok(app)
}

/// Opens or creates all of the column families in the rocksdb connection.
/// Will panic on fail.
fn must_init_db(dbpath: String) -> DB {
    let opts = {
        use rocksdb::DBOptions;
        let mut opts = DBOptions::default();
        opts.create_if_missing(true);
        opts
    };

    // List column families used outside of default.
    use habits::constants::*;
    let column_families = vec![CF_HABITS];

    let mut all = vec!["default"];
    all.extend(column_families.clone());

    match DB::open_cf(opts, &dbpath, all) {
        Ok(db) => db,
        Err(e) => {
            info!("Creating new db, {}", e);

            let mut db = DB::open_default(&dbpath).expect("New database: open default");

            for cf in column_families {
                db.create_cf(cf)
                    .expect(format!("Create {} column family", cf).as_str());
            }

            db
        }
    }
}
