#[cfg(test)]
mod tests {
    use std::env;

    use serde_json::json;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        // await setDoc(doc(db, "cities", "LA"), {
        //   name: "Los Angeles",
        //   state: "CA",
        //   country: "USA"
        // });
        //
        // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/createDocument>
        // `POST https://firestore.googleapis.com/v1/{parent=projects/*/databases/*/documents/**}/{collectionId}`
        // OAuth Scope:
        // - https://www.googleapis.com/auth/datastore
        // - https://www.googleapis.com/auth/cloud-platform

        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)"; // env::var("DATABASE_ID")?;
        let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
        let collection_id = "cities";
        let client = reqwest::Client::new();
        let url = format!(
            "https://firestore.googleapis.com/v1/projects/{}/databases/{}/documents/{}",
            project_id, database_id, collection_id
        );
        let name = format!(
            "projects/{}/databases/{}/documents/{}",
            project_id, database_id, collection_id
        );
        let body = json!({
          "name": name,
          "fields": {
            "foo": {
              "stringValue": "bar"
            }
          }
        });
        let response = client
            .post(url)
            .header("X-Goog-User-Project", project_id)
            .header("Authorization", format!("Bearer {}", bearer_token))
            .body(serde_json::to_string(&body)?)
            .send()
            .await?;
        let status = response.status();
        // assert_eq!(response.bytes().await?, "");
        assert_eq!(status, 200);
        Ok(())
    }
}
