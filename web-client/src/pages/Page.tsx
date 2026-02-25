import Header from "@/components/Header"
import { Separator } from "@/components/ui/separator"

interface AuthProviderProps {
    children: React.ReactNode
    title: string
    desc?: string
}

export function Page({ children, title, desc }: AuthProviderProps) {
    return (
        <div className="p-6 space-y-4">
            <Header />
            <h1 className="text-xl">{title}</h1>
            <p>{desc}</p>
            <Separator />
            {children}
        </div>
    )
}
