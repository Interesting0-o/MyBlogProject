const apiBaseUrl = (import.meta.env.VITE_API_BASE_URL || '/api').replace(/\/+$/, '')
const authBaseUrl = `${apiBaseUrl}/admin`

async function request(path: string, options: RequestInit = {}): Promise<Response> {
  return fetch(`${authBaseUrl}${path}`, options)
}

export async function login(username: string, password: string): Promise<void> {
  const response = await request('/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, password }),
  })

  if (!response.ok) {
    let message = `登录失败：${response.status}`

    try {
      const payload = (await response.json()) as { error?: string }

      if (payload.error) {
        message = payload.error
      }
    } catch {
      // 保留默认错误信息
    }

    throw new Error(message)
  }
}

export async function logout(): Promise<void> {
  await request('/logout', { method: 'POST' })
}

export async function checkSession(): Promise<boolean> {
  try {
    const response = await request('/session')
    return response.ok
  } catch {
    return false
  }
}
