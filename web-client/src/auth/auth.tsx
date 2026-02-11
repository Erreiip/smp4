import type { User } from '@/interfaces/User';
import { createContext, useContext, useEffect, useState, type Dispatch, type SetStateAction } from 'react'

interface AuthContextType {
  user: User | null
  login: (username: string, password: string) => Promise<void>
  register: (email: string, password: string, username: string, setErrors: Dispatch<SetStateAction<{ username?: string | undefined; email?: string | undefined; password?: string | undefined; }>>) => Promise<void>
  fetchUserData: () => Promise<void>
  logout: () => Promise<void>
  loading: boolean
}
const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8888';

const AuthContext = createContext<AuthContextType | undefined>(undefined)

interface AuthProviderProps {
  children: React.ReactNode
}

export function AuthProvider({ children }: AuthProviderProps) {
  const [user, setUser] = useState<User | null>(null)
  const [loading, setLoading] = useState(true)

  async function login(username: string, password: string): Promise<void> {
    setLoading(true)

    try {
      const credentials = btoa(`${username}:${password}`)

      const res = await fetch(`${API_URL}/auth/login`, {
        method: 'POST',
        headers: {
          'Authorization': `Basic ${credentials}`,
          'Content-Type': 'application/json',
        },
        credentials: 'include',
      })


      const data = await res.json()

      if (!res.ok) {
        throw new Error(data?.error ?? 'Login failed')
      }

      const { token } = data

      localStorage.setItem('token', token)
      fetchUserData()
    } finally {
      setLoading(false)
    }
  }

  async function register(email: string, password: string, username: string, setErrors: Dispatch<SetStateAction<{ username?: string | undefined; email?: string | undefined; password?: string | undefined; }>>) {
    const res = await fetch(`${API_URL}/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password, email }),
      credentials: 'include',
    });
    let data = await res.json();

    if (!res.ok) {
      const { error } = data;
      if (error) {
        setErrors({ username: `* ${error}`, email: `${error}` });
      }
      throw new Error('Registration failed');
    }

    const { token } = data;
    localStorage.setItem("token", token)
  }

  async function logout() {
    localStorage.removeItem("token")
    setUser(null)
  }

  async function fetchUserData(): Promise<void> {
    const token = localStorage.getItem('token')
    if (!token) {
      setUser(null)
      setLoading(false)
      return
    }

    try {
      const res = await fetch(`${API_URL}/user`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      })

      if (!res.ok) throw new Error('Not authenticated')

      const data = await res.json()
      setUser(data)
    } catch {
      setUser(null)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchUserData()
  }, [])


  return (
    <AuthContext.Provider
      value={{
        user,
        login,
        register,
        logout,
        fetchUserData,
        loading
      }}
    >
      {children}
    </AuthContext.Provider>
  )
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}
