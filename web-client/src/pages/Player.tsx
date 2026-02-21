import FilePicker from "@/components/FilePicker";
import Header from "@/components/Header"

export default function Player() {
    return (
        <div className="p-6 space-y-4">
            <Header />
            <FilePicker />
        </div>
    )
}
