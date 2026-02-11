import { Routes, Route } from "react-router-dom"
import { ThemeProvider } from "@/components/ui/theme-provider"
import Login from "./pages/Login"
import Register from "./pages/Register"
import Player from "./pages/Player"
import ProtectedRoute from "./components/ProtectedRoute"
import { AuthProvider } from "./auth/auth"
import Profil from "./pages/Profil"

export default function App() {
  return (
    <AuthProvider>
      <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route path="/register" element={<Register />} />

          <Route path="/" element={<Player />} />
          <Route path="/player" element={<Player />} />
          <Route
            path="/profil"
            element={
              <ProtectedRoute>
                <Profil />
              </ProtectedRoute>
            }
          />
        </Routes>
      </ThemeProvider>
    </AuthProvider>
  )
}
