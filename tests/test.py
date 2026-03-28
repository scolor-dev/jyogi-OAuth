import os
from pathlib import Path
from dotenv import load_dotenv

# テスト関数 import
from api.test_health import test_health
from api.test_auth import (
    test_login,
    test_me,
    test_refresh,
    test_logout,
)

# =========================
# 環境変数読み込み
# =========================
env_path = Path(__file__).parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("BASE_URL")
DEBUG = os.getenv("DEBUG", "true").lower() == "true"

if not BASE_URL:
    raise RuntimeError("BASE_URL is not set in tests/.env")


# =========================
# 共通出力
# =========================
def print_result(name: str, success: bool, detail: str = ""):
    status = "OK" if success else "FAIL"
    print(f"[{status}] {name}")
    if detail:
        print(f"  -> {detail}")


# =========================
# 実行
# =========================
def main():
    print("=== API TEST START ===")

    results = []

    # health
    results.append(test_health(BASE_URL, print_result, DEBUG))

    # auth
    results.append(test_login(BASE_URL, print_result, DEBUG))
    results.append(test_me(BASE_URL, print_result, DEBUG))
    results.append(test_refresh(BASE_URL, print_result, DEBUG))
    results.append(test_logout(BASE_URL, print_result, DEBUG))

    # 集計
    success_count = sum(results)
    total = len(results)

    print("\n=== RESULT ===")
    print(f"{success_count}/{total} passed")

    if success_count != total:
        exit(1)


if __name__ == "__main__":
    main()