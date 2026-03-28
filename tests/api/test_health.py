import requests


# =========================
# DEBUG関数（authと同じ）
# =========================
def debug_http(label: str, res: requests.Response):
    req = res.request

    print(f"\n--- {label} ---")

    # Request
    print(">>> REQUEST")
    print(f"{req.method} {req.url}")

    print("[Headers]")
    for k, v in req.headers.items():
        print(f"{k}: {v}")

    if req.body:
        try:
            print("[Body]")
            print(req.body.decode() if isinstance(req.body, bytes) else req.body)
        except Exception:
            print(req.body)

    # Response
    print("\n<<< RESPONSE")
    print(f"Status: {res.status_code}")

    print("[Headers]")
    for k, v in res.headers.items():
        print(f"{k}: {v}")

    print("[Cookies]")
    for k, v in res.cookies.items():
        print(f"{k}: {v}")

    try:
        print("[Body]")
        print(res.json())
    except Exception:
        print(res.text)

    print("--- END ---\n")


# =========================
# TEST
# =========================
def test_health(base_url, print_result, debug=False):
    name = "GET /api/v1/health"

    try:
        res = requests.get(f"{base_url}/api/v1/health")

        if debug:
            debug_http("HEALTH", res)

        if res.status_code != 200:
            print_result(name, False, f"status={res.status_code}")
            return False

        # 空レスポンス許容
        if not res.text:
            print_result(name, True, "empty response")
            return True

        # JSONならチェック
        try:
            body = res.json()
            if body.get("data", {}).get("status") == "ok":
                print_result(name, True)
                return True
            else:
                print_result(name, False, f"invalid body={body}")
                return False
        except Exception:
            print_result(name, False, f"non-json response: {res.text}")
            return False

    except Exception as e:
        print_result(name, False, str(e))
        return False