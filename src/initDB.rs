use sqlx::PgPool;

use crate::models::Diary;

pub async fn defaults(pool : &PgPool){
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS diary (
            id BIGSERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            address TEXT NOT NULL,
            UNIQUE (name, address)  
        );
    "#;
    let create_table_query_entries = r#"
        CREATE TABLE IF NOT EXISTS entries (
            id BIGSERIAL PRIMARY KEY,
            entry TEXT,
            diary_id BIGINT UNIQUE,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP  
        );
    "#;
    sqlx::query(create_table_query)
        .execute(pool)
        .await
        .expect("Failed to create table");
    sqlx::query(create_table_query_entries)
        .execute(pool)
        .await
        .expect("Failed to create table");
}

pub async fn new_entry(pool : &PgPool, diary : &Diary) -> i64 {
    let insert_query = r#"
        INSERT INTO diary (name, address)
        VALUES ($1, $2)
        ON CONFLICT (name, address) DO UPDATE
            SET name = EXCLUDED.name
        RETURNING id;
    "#;
    let id: Option<i64> = sqlx::query_scalar(insert_query)
                        .bind(&diary.name)
                        .bind(&diary.address)
                        .fetch_optional(pool)
                        .await
                        .expect("Failed to insert new entry");

    id.unwrap_or(-1)
}

pub async fn new_text_entry(pool : &PgPool, diary_id : i64)  {
    let insert_query = r#"
        INSERT INTO entries (diary_id)
        VALUES ($1)
        ON CONFLICT (diary_id) DO NOTHING;
    "#;

    sqlx::query(insert_query)
        .bind(&diary_id)
        .execute(pool)
        .await
        .expect("Failed to insert new entry");
    
}

pub async fn get_entry(pool : &PgPool, id: i64) -> Result<String, String> {
    let select_query = r#"
        SELECT entry
        FROM entries
        WHERE id = $1;
    "#;

    let entry : Option<Option<String>> = sqlx::query_scalar(select_query)
                            .bind(&id)
                            .fetch_optional(pool)
                            .await
                            .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(entry.flatten().unwrap_or_else(String::new))
}

pub async fn update_entry(pool : &PgPool, id: i64, text: &String) {
    let update_query = r#"
        UPDATE entries
        SET entry = $1
        WHERE id = $2;
    "#;

    sqlx::query(update_query)
        .bind(&text)
        .bind(&id)
        .execute(pool)
        .await
        .expect("Failed to update entry");
}
