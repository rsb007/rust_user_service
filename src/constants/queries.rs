pub const KEYSPACE: &str ="CREATE KEYSPACE IF NOT EXISTS user_ks WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";

pub const EVENT_TABLE: &str = "CREATE TABLE IF NOT EXISTS user_ks.user_events(user_id uuid PRIMARY KEY , \
     user_event text);";

pub const STATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS user_ks.user_state (user_id uuid PRIMARY KEY , \
     user_state text);";