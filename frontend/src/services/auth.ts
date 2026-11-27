const apiBaseUrl = (import.meta.env.VITE_API_BASE_URL || '/api').replace(/\/+$/, '')
const authBaseUrl = `${apiBaseUrl}/admin`

async function request(path: string, options: RequestInit = {}): Promise<Response> {
  return fetch(`${authBaseUrl}${path}`, options)
}

function fallbackMessage(status: number): string {
  if (status === 401) return '用户名或密码错误'
  if (status === 403) return '没有访问权限'
  if (status === 429) return '尝试过于频繁，请稍后再试'
  if (status === 503) return '登录暂不可用：服务端尚未配置管理员凭证'
  if (status >= 500) return '服务暂时不可用，请稍后再试'
  return '登录失败，请稍后再试'
}

export async function login(username: string, password: string): Promise<void> {
  let response: Response

  try {
    response = await request('/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    })
  } catch {
    throw new Error('网络连接失败，请检查网络后重试')
  }

  if (!response.ok) {
    let message = fallbackMessage(response.status)

    try {
      const payload = (await response.json()) as { error?: string }

      if (payload.error) {
        message = payload.error
      }
    } catch {
      // 响应体不是 JSON 时使用按状态码映射的提示
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
