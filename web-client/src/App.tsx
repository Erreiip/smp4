import { Routes, Route } from "react-router-dom"
import { ThemeProvider } from "@/components/ui/theme-provider"
import Login from "./pages/Login"
import Register from "./pages/Register"
import Uploader from "./pages/Uploader"
import ProtectedRoute from "./components/ProtectedRoute"
import { AuthProvider } from "./auth/auth"
import { TooltipProvider } from "@/components/ui/tooltip";
import Profil from "./pages/Profil"
import Player from "./pages/Player"

export default function App() {
  return (
    <AuthProvider>
      <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
        <TooltipProvider>
          <Routes>
            <Route path="/login" element={<Login />} />
            <Route path="/register" element={<Register />} />

            <Route path="/upload" element={
              <ProtectedRoute>
                <Uploader />
              </ProtectedRoute>
            } />
            <Route path="/" element={<Player />} />
            <Route path="/player" element={<Player />} />
            <Route
              path="/profil"
              element={
                <ProtectedRoute>
                  <Profil />
                </ProtectedRoute>
              } />
          </Routes>
        </TooltipProvider>
      </ThemeProvider>
    </AuthProvider>
  )
}
