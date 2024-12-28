# fcm-send

```console
GOOGLE_APPLICATION_CREDENTIALS=/path/to/credential.json \
  fcm-send \
    --body '記事 2024-12-27 が追加されました' \
    --data 'url=https://blog.bouzuya.net/2024/12/27' \
    --icon 'https://bouzuya.net/images/favicon.png' \
    --title 'blog.bouzuya.net からのお知らせ' \
    --token 'YOUR_REGISTRATION_TOKEN'
```
