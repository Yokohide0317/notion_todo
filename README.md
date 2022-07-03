# notion-todo

コマンドラインから、NotionのデータベースにToDoを投げる

## init setting

### Create an integration

1. Go to ("Notion Developers")[https://www.notion.so/my-integrations]

2. Get "Access Token"

3. (Create | Choose) Database You want to use as TODO list

4. Get Database ID
- Open the Database
- Click "Share" to get URL
- ex: https://www.notion.so/username/＜databaseid＞?v=hogehoge

### Setting env

```

export NOTION_ACCESS_TOKEN=<Your Notion's Access Token>
export NOTION_DATABASE_ID=<Your Notion's Database ID to Add todo>


```

## Usage

```

# todoの追加
todo 卒研日誌 仕上げる

# 開発中
todo list

```
