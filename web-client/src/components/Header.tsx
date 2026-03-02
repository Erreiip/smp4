import { ModeToggle } from "@/components/ModeToggle"
import { useAuth } from "@/auth/auth"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "./ui/dropdown-menu"
import { Avatar, AvatarFallback } from "./ui/avatar"
import { useNavigate } from "react-router-dom"
import { Button } from "./ui/button"
import { Badge } from "./ui/badge"
import { ButtonGroup } from "./ui/button-group"
import { IoLogOut } from "react-icons/io5"

export default function Header() {
    const { user, logout } = useAuth()
    const navigate = useNavigate()

    return (
        <div className="flex justify-between items-center p-4">
            <Badge onClick={() => navigate("/")}><h1 className="text-2xl font-bold">S-File</h1></Badge>
            <ButtonGroup>
                <Button variant="outline" onClick={() => navigate("/upload")} disabled={!user}>File Signer</Button>
                <Button variant="outline" onClick={() => navigate("/")}>File Verifier</Button>
            </ButtonGroup>
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
                            <DropdownMenuItem onClick={() => navigate("/profil")}>
                                Profil
                            </DropdownMenuItem>
                            <DropdownMenuItem onClick={() => logout()} className="text-red-400">
                                Logout <IoLogOut className="text-red-400" />
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
