import FilePicker from "@/components/FilePicker";
import { Page } from "./Page";
import { Card, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

export default function Player() {
    return (
        <Page title="Player" desc="Upload a file you wish to verify">
            <div className="min-h-xl flex items-center gap-6">
                <FilePicker />
                <Card className="flex-1 h-full">
                    <CardHeader>
                        <CardTitle>Uploader you file here</CardTitle>
                        <CardDescription>Uploader you file here</CardDescription>
                    </CardHeader>
                </Card>
            </div >
        </Page>
    )
}
