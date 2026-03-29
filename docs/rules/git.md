# ブランチ・コミット規則

---

## ブランチ

---

### 形式
```
type/{issue番号}-{概要}
```
例：
```
feature/123-add-login
fix/456-crash
```

---

### type
- `feature` 機能追加
- `fix` バグ修正
- `refactor` 内部改善
- `document` ドキュメント
- `chore` その他

---

## コミット

---

### 形式
```
type: 概要
```
例：
```
feat: ログイン機能追加
fix: クラッシュ修正
refactor: 構造整理
```

---

### type
- `feat` 機能追加
- `fix` バグ修正
- `refactor` 内部改善
- `docs` ドキュメント
- `chore` その他

---

## ルール

- 1ブランチ = 1Issue
- 1コミット = 1目的
- 短く簡潔に書く