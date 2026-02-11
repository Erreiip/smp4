import { Navigate } from "react-router-dom"
import { useAuth } from "@/auth/auth"
import type { ReactNode } from "react"
import { Spinner } from "./ui/spinner"

export default function ProtectedRoute({
    children,
}: {
    children: ReactNode
}) {
    const { user, loading } = useAuth()

    if (loading) {
        return (
            <div className="fixed inset-0 z-50 flex items-center justify-center bg-background/70 backdrop-blur-sm">
                <div className="flex flex-col items-center">
                    <Spinner className="h-20 w-20 text-primary" />
                    <span className="mt-4 text-lg font-medium text-primary">
                        Loading...
                    </span>
                </div>
            </div>
        )
    }


    if (!user) {
        return <Navigate to="/" replace />
    }

    return <>{children}</>
}
