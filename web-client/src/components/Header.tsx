import { ModeToggle } from "@/components/ModeToggle"
import { useAuth } from "@/auth/auth"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "./ui/dropdown-menu"
import { Avatar, AvatarFallback } from "./ui/avatar"
import { useNavigate } from "react-router-dom"
import { Button } from "./ui/button"

export default function Header() {
    const { user, logout } = useAuth()
    const navigate = useNavigate()

    return (
        <div className="flex justify-between items-center p-4">
            <h1 className="text-2xl font-bold">Player</h1>
            <div className="flex items-center space-x-2">
                <ModeToggle />

                {user ? (
                    <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                            <Avatar>
                                <AvatarFallback>{user.username?.charAt(0).toUpperCase()}</AvatarFallback>
                            </Avatar>
                        </DropdownMenuTrigger>

                        <DropdownMenuContent align="end">
                            <DropdownMenuItem onClick={() => logout()}>
                                Logout
                            </DropdownMenuItem>
                            <DropdownMenuItem onClick={() => navigate("/profil")}>
                                Profil
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                ) :
                    <Button
                        variant="outline"
                        onClick={() => navigate("/login")}>
                        Login
                    </Button>
                }
            </div>
        </div>
    )

}
