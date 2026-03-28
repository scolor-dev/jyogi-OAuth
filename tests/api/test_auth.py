import requests
import time


def unique_identifier():
    return f"user_{int(time.time() * 1000)}"


# =========================
# DEBUG関数
# =========================
def debug_http(label: str, res: requests.Response, session=None):
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

    # Session Cookie（重要）
    if session:
        print("\n[Session Cookies]")
        print(session.cookies.get_dict())

    print("--- END ---\n")


# =========================
# API
# =========================
def signup_user(base_url, identifier, password):
    return requests.post(
        f"{base_url}/api/v1/auth/signup",
        json={
            "identifier": identifier,
            "password": password,
            "display_name": identifier
        }
    )


def login_user(session, base_url, identifier, password):
    return session.post(
        f"{base_url}/api/v1/auth/login",
        json={
            "identifier": identifier,
            "password": password
        }
    )


# =========================
# TESTS
# =========================
def test_login(base_url, print_result, debug=False):
    name = "POST /api/v1/auth/login"

    try:
        identifier = unique_identifier()
        password = "password"

        signup_res = signup_user(base_url, identifier, password)
        if debug:
            debug_http("SIGNUP", signup_res)

        if signup_res.status_code not in (200, 201):
            print_result(name, False, f"signup failed: {signup_res.status_code}")
            return False

        session = requests.Session()
        res = login_user(session, base_url, identifier, password)

        if debug:
            debug_http("LOGIN", res, session)

        if res.status_code != 200:
            print_result(name, False, f"status={res.status_code}")
            return False

        body = res.json()

        if "access_token" not in body:
            print_result(name, False, f"no token: {body}")
            return False

        if "refresh_token" not in session.cookies:
            print_result(name, False, "no refresh_token cookie")
            return False

        print_result(name, True)
        return True

    except Exception as e:
        print_result(name, False, str(e))
        return False


def test_me(base_url, print_result, debug=False):
    name = "GET /api/v1/auth/me"

    try:
        identifier = unique_identifier()
        password = "password"

        signup_user(base_url, identifier, password)

        session = requests.Session()
        res = login_user(session, base_url, identifier, password)

        token = res.json().get("access_token")

        res = session.get(
            f"{base_url}/api/v1/auth/me",
            headers={"Authorization": f"Bearer {token}"}
        )

        if debug:
            debug_http("ME", res, session)

        if res.status_code != 200:
            print_result(name, False, f"status={res.status_code}")
            return False

        body = res.json()

        if body.get("identifier") != identifier:
            print_result(name, False, f"unexpected user: {body}")
            return False

        print_result(name, True)
        return True

    except Exception as e:
        print_result(name, False, str(e))
        return False


def test_refresh(base_url, print_result, debug=False):
    name = "POST /api/v1/auth/refresh"

    try:
        identifier = unique_identifier()
        password = "password"

        signup_user(base_url, identifier, password)

        session = requests.Session()
        res = login_user(session, base_url, identifier, password)

        old_token = res.json().get("access_token")

        res = session.post(f"{base_url}/api/v1/auth/refresh")

        if debug:
            debug_http("REFRESH", res, session)

        if res.status_code != 200:
            print_result(name, False, f"status={res.status_code}")
            return False

        new_token = res.json().get("access_token")

        if not new_token:
            print_result(name, False, "no new token")
            return False

        if new_token == old_token:
            print_result(name, False, "token not refreshed")
            return False

        print_result(name, True)
        return True

    except Exception as e:
        print_result(name, False, str(e))
        return False


def test_logout(base_url, print_result, debug=False):
    name = "POST /api/v1/auth/logout"

    try:
        identifier = unique_identifier()
        password = "password"

        signup_user(base_url, identifier, password)

        session = requests.Session()
        login_user(session, base_url, identifier, password)

        res = session.post(f"{base_url}/api/v1/auth/logout")

        if debug:
            debug_http("LOGOUT", res, session)

        if res.status_code != 200:
            print_result(name, False, f"status={res.status_code}")
            return False

        res = session.post(f"{base_url}/api/v1/auth/refresh")

        if debug:
            debug_http("REFRESH_AFTER_LOGOUT", res, session)

        if res.status_code == 200:
            print_result(name, False, "refresh should fail after logout")
            return False

        print_result(name, True)
        return True

    except Exception as e:
        print_result(name, False, str(e))
        return False