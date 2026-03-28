import type { User } from '../types/user'

export type LoginResponse = {
  user: User
  access_token: string
}

type ApiErrorBody = {
  message?: string
  detail?: string
}

// =========================
// Login
// =========================
export async function loginApi(
  identifier: string,
  password: string
): Promise<LoginResponse> {
  const res = await fetch('/api/v1/auth/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify({ identifier, password }),
  })

  if (!res.ok) {
    const body: ApiErrorBody = await res.json().catch(() => ({}))
    throw new ApiError(res.status, body.message ?? body.detail ?? 'ログインに失敗しました')
  }

  const data = (await res.json()) as { access_token: string }

  if (!data.access_token) {
    throw new ApiError(500, 'アクセストークンが取得できませんでした')
  }

  const user = await getMeApi(data.access_token)

  return { user, access_token: data.access_token }
}

// =========================
// Signup
// =========================
export async function signupApi(
  identifier: string,
  displayName: string,
  password: string
): Promise<LoginResponse> {
  const res = await fetch('/api/v1/auth/signup', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify({
      identifier,
      display_name: displayName,
      password,
    }),
  })

  if (!res.ok) {
    const body: ApiErrorBody = await res.json().catch(() => ({}))
    throw new ApiError(res.status, body.message ?? body.detail ?? '新規登録に失敗しました')
  }

  // signup成功 → loginで補完
  try {
    return await loginApi(identifier, password)
  } catch {
    throw new ApiError(
      0,
      '登録は完了しましたが自動ログインに失敗しました。ログイン画面からサインインしてください。'
    )
  }
}

// =========================
// Logout
// =========================
export async function logoutApi(): Promise<void> {
  await fetch('/api/v1/auth/logout', {
    method: 'POST',
    credentials: 'include',
  })
}

// =========================
// Me
// =========================
export async function getMeApi(accessToken: string): Promise<User> {
  const res = await fetch('/api/v1/auth/me', {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
    credentials: 'include',
  })

  if (!res.ok) {
    throw new ApiError(res.status, 'ユーザー情報の取得に失敗しました')
  }

  const raw = (await res.json()) as {
    user_uuid: string
    display_name: string
    identifier: string
  }

  return {
    uuid: raw.user_uuid,
    identifier: raw.identifier,
    display_name: raw.display_name,
  }
}

// =========================
// Error
// =========================
export class ApiError extends Error {
  constructor(
    public readonly status: number,
    message: string
  ) {
    super(message)
    this.name = 'ApiError'
  }
}