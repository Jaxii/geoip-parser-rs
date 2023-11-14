

mod database {

    pub struct GeoIPDatabase {
        database: Database
    }

    impl GeoIPDatabase {

        pub fn initialize() -> Result<GeoIPDatabase, DbErr> {

            let db = Database::connect("sqlite::memory:");

            Ok(());

        }

        pub fn new_entry(ip_entry: IPAddressAllocation) {
            //todo: save
        }
    }
    

}