use rusqlite::{params, Connection, Result, Error};

const TABLES: [&str; 2] = [
    "CREATE TABLE IF NOT EXISTS users (
    user_id INT PRIMARY KEY,
    available_slots INT NOT NULL,
    subscription_start INT NOT NULL,
    subscription_expiry INT NOT NULL
)",
    "CREATE TABLE IF NOT EXISTS appointments (
    UUID INT PRIMARY KEY,
    locator INT NOT NULL,
    encrypted_blob BLOB NOT NULL,
    to_self_delay INT NOT NULL,
    user_signature BLOB NOT NULL,
    start_block INT NOT NULL,
    user_id INT NOT NULL,
    FOREIGN KEY(user_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
)"
];

struct User {
    user_id: i64,
    available_slots: i64,
    subscription_start: i64,
    subscription_expiry: i64,
}

struct Appointment {
    uuid: i64,
    locator: i64,
    encrypted_blob: Vec<u8>,
    to_self_delay: i64,
    users_signature: Vec<u8>,
    start_block: i64,
    user_id: i64,
}

struct Database {
    conn: Connection
}

impl Database {
    fn new(db_path: String) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Self::create_tables(&conn)?;
        Ok(Self { conn })
    }

    fn create_tables(conn: &Connection) -> Result<()> {
        for table in TABLES.iter() {
            conn.execute(table, params![])?;
        }
        Ok(())
    }

    fn create_user(&self, user_id: i32, available_slots: i32, subscription_start: i32, subscription_expiry: i32) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO users (user_id, available_slots, subscription_start, subscription_expiry) VALUES (?1, ?2, ?3, ?4)",
            params![user_id, available_slots, subscription_start, subscription_expiry]
        )
    }

    fn read_user(&self, user_id: i32) -> Result<Option<(i32, i32, i32, i32)>> {
        let mut stmt = self.conn.prepare("SELECT user_id, available_slots, subscription_start, subscription_expiry FROM users WHERE user_id = ?1")?;
        let mut user_iter = stmt.query_map(params![user_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))?;
        user_iter.next().transpose()
    }

    fn read_users_by_subscription_expiry(&self, subscription_expiry: i32) -> Result<Vec<(i32, i32, i32, i32)>> {
        let mut stmt = self.conn.prepare("SELECT user_id, available_slots, subscription_start, subscription_expiry FROM users WHERE subscription_expiry = ?1")?;
        let user_iter = stmt.query_map(params![subscription_expiry], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))?;
        user_iter.collect()
    }
    fn read_users_by_available_slots(&self, available_slots: i32) -> Result<Vec<(i32, i32, i32, i32)>> {
        let mut stmt = self.conn.prepare("SELECT user_id, available_slots, subscription_start, subscription_expiry FROM users WHERE available_slots = ?1")?;
        let user_iter = stmt.query_map(params![available_slots], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))?;
        user_iter.collect()
    }

    fn update_user(&self, user_id: i32, available_slots: i32, subscription_start: i32, subscription_expiry: i32) -> Result<usize> {
        self.conn.execute(
            "UPDATE users SET available_slots = ?2, subscription_start = ?3, subscription_expiry = ?4 WHERE user_id = ?1",
            params![user_id, available_slots, subscription_start, subscription_expiry]
        )
    }
    
    fn delete_user(&self, user_id: i32) -> Result<usize> {
        self.conn.execute("DELETE FROM users WHERE user_id = ?1", params![user_id])
    }

    fn create_appointment(&self, uuid: i32, locator: i32, encrypted_blob: &[u8], to_self_delay: i32, user_signature: &[u8], start_block: i32, user_id: i32) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO appointments (uuid, locator, encrypted_blob, to_self_delay, user_signature, start_block, user_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![uuid, locator, encrypted_blob, to_self_delay, user_signature, start_block, user_id]
        )
    }

    fn read_appointment(&self, uuid: i32) -> Result<Option<(i32, i32, Vec<u8>, i32, Vec<u8>, i32, i32)>> {
        let mut stmt = self.conn.prepare("SELECT uuid, locator, encrypted_blob, to_self_delay, user_signature, start_block, user_id FROM appointments WHERE uuid = ?1")?;
        let mut appointment_iter = stmt.query_map(params![uuid], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?)))?;
        appointment_iter.next().transpose()
    }

    fn read_appointments_by_user_id(&self, user_id: i32) -> Result<Vec<(i32, i32, Vec<u8>, i32, Vec<u8>, i32, i32)>> {
        let mut stmt = self.conn.prepare("SELECT uuid, locator, encrypted_blob, to_self_delay, user_signature, start_block, user_id FROM appointments WHERE user_id = ?1")?;
        let appointment_iter = stmt.query_map(params![user_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?)))?;
        appointment_iter.collect()
    }
    
    fn update_appointment(&self, uuid: i32, locator: i32, encrypted_blob: &[u8], to_self_delay: i32, user_signature: &[u8], start_block: i32, user_id: i32) -> Result<usize> {
        self.conn.execute(
            "UPDATE appointments SET locator = ?2, encrypted_blob = ?3, to_self_delay = ?4, user_signature = ?5, start_block = ?6, user_id = ?7 WHERE uuid = ?1",
            params![uuid, locator, encrypted_blob, to_self_delay, user_signature, start_block, user_id]
        )
    }

    fn delete_appointment(&self, uuid: i32) -> Result<usize> {
        self.conn.execute("DELETE FROM appointments WHERE uuid = ?1", params![uuid])
    }

}

fn main() -> Result<(), Error> {
    let db = Database::new("teos_db.sql3".to_string())?;
    db.create_user(1, 5, 1646438400, 1677974400)?;
    
    let user = db.read_user(1)?;
    println!("User: {:?}", user);

    let users_by_available_slots = db.read_users_by_available_slots(5)?;
    println!("Users by available slots: {:?}", users_by_available_slots);
    
    let users_by_subscription_expiry = db.read_users_by_subscription_expiry(1677974400)?;
    println!("Users by subscription expiry: {:?}", users_by_subscription_expiry);

    db.update_user(1, 10, 1646438400, 1680518400)?;

    let encrypted_blob = b"encrypted_blob".to_vec();
    let user_signature = b"user_signature".to_vec();
    db.create_appointment(1, 1, &encrypted_blob, 144, &user_signature, 123456, 1)?;
    
    let appointment = db.read_appointment(1)?;
    println!("Appointment: {:?}", appointment);

    let appointments_by_user_id = db.read_appointments_by_user_id(1)?;
    println!("Appointments by user id: {:?}", appointments_by_user_id);
    
    let new_encrypted_blob = b"new_encrypted_blob".to_vec();
    let new_user_signature = b"new_user_signature".to_vec();
    db.update_appointment(1, 2, &new_encrypted_blob, 288, &new_user_signature, 234567, 1)?;
    
    // db.delete_appointment(1)?;
    // db.delete_user(1)?;
    
    Ok(())
}

