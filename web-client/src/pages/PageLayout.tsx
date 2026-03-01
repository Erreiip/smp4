import Header from "@/components/Header"
import { Separator } from "@/components/ui/separator"

interface AuthProviderProps {
    children: React.ReactNode
    title: string
    desc: string | TrustedHTML
}

export function PageLayout({ children, title, desc }: AuthProviderProps) {
    return (
        <div className="p-6 space-y-4">
            <Header />
            <h1 className="text-4xl">{title}</h1>
            <p dangerouslySetInnerHTML={{ __html: desc }} />
            <Separator />
            {children}
        </div>
    )
}
