import { PageLayout } from "./PageLayout";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { useEffect, useState } from "react";
import FilePicker from "@/components/FilePicker";
import { Button } from "@/components/ui/button";
import { Spinner } from "@/components/ui/spinner";
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Textarea } from "@/components/ui/textarea";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { FaDownload } from "react-icons/fa";

export default function Player() {
    const [file, setFile] = useState<File | null>();
    const [loading, setLoading] = useState<boolean>(false)
    const [allowDownload, setAllowDownload] = useState<boolean>(false)
    const [error, setError] = useState<string | null>(null)
    const [fieldData, setFieldData] = useState<{
        author?: string
        email?: string
        oid?: string
        description?: string
        link?: string
        license?: string
    }>({})
    const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8888';

    const handleFileDrop = (files: File[]) => {
        if (files.length != 1)
            throw new Error('Single file required')
        let file = files[0]
        setFile(file)
    };

    const uploadFile = async () => {
        const token = localStorage.getItem("token");

        if (!file) return;
        try {
            setLoading(true)
            const data = new FormData();

            const jsonBlob = new Blob(
                [JSON.stringify({
                    name: `${file.name}`,
                })],
                { type: "application/json" }
            );

            data.append("json", jsonBlob);
            data.append("file", file);

            const response = await fetch(`${API_URL}/file/verify`, {
                method: "POST",
                headers: {
                    Authorization: `Bearer ${token}`,
                },
                body: data,
            });

            if (!response.ok) {
                setError("* It looks like you tried to upload a non SFile or a modified SFile since it was signed.")
                throw new Error("Upload failed");
            }
            const rep = await response.json()
            setAllowDownload(true)
            setFieldData(rep)

        } catch (error) {
            console.error(error);
        }
        finally {
            setLoading(false)
        }
    };

    const downloadOriginalFile = async () => {
        const token = localStorage.getItem("token");

        if (!file) return;
        try {
            const data = new FormData();

            const jsonBlob = new Blob(
                [JSON.stringify({
                    name: `${file.name}`,
                })],
                { type: "application/json" }
            );

            data.append("json", jsonBlob);
            data.append("file", file);

            const response = await fetch(`${API_URL}/file/decode`, {
                method: "POST",
                headers: {
                    Authorization: `Bearer ${token}`,
                },
                body: data,
            });

            if (!response.ok) {
                setError("* Failed to download original file")
                throw new Error("Upload failed");
            }

            const blob = await response.blob();

            let filename = "downloaded-file";

            const disposition = response.headers.get("Content-Disposition");
            if (disposition && disposition.includes("filename=")) {
                filename = disposition
                    .split("filename=")[1]
                    .replace(/"/g, "");
            }

            const link = document.createElement("a");
            link.href = URL.createObjectURL(blob);
            link.download = filename;
            link.click();
            URL.revokeObjectURL(link.href);

        } catch (error) {
            console.error(error);
        }
        finally {
            setLoading(false)
        }
    };

    useEffect(() => {
        if (!file) {
            setAllowDownload(false)
            setError(null)
            setFieldData({})
        }
    }, [file])


    return (
        <PageLayout title="File Verifier" desc="
        Please upload a signed file here that you wish to verify. The information on the right corresponds to the information contained within the file.
        <br />
        Sfile guarantees that this information matches the file and has not been altered since the creation of this secure file.
        ">
            <div className="min-h-xl flex items-stretch gap-6">
                <Card className="w-4xl flex-3">
                    <CardHeader>
                        <CardTitle>Upload your file here</CardTitle>
                    </CardHeader>
                    <CardDescription className="flex gap-6 flex-col px-6 [&>div]:w-full">
                        <FilePicker handleCallBack={handleFileDrop} file={file} setFile={setFile} />
                        <p className="text-sm text-red-400">{error}</p>
                    </CardDescription>
                    {(file) &&
                        <CardFooter className="flex gap-6">
                            <Button className="flex-1" variant="outline" id="clear-btn" onClick={() => setFile(null)}>Clear</Button>
                            <Button className="flex-1" onClick={() => { uploadFile() }}>{loading && <Spinner />}Verify</Button>
                        </CardFooter>
                    }
                    {(allowDownload) &&
                        <CardContent onClick={downloadOriginalFile} className="border-2 flex-col border-dashed flex items-center align-middle gap-3 m-6 p-9 cursor-pointer hover:bg-muted transition">
                            <FaDownload size={20} />
                            <span>Download original file</span>
                        </CardContent>
                    }

                </Card>
                <Card className="flex-2">
                    <CardHeader>
                        <CardTitle>File Information</CardTitle>
                        <Separator />
                        <CardDescription>
                            <FieldGroup className="mt-6">
                                <Field>
                                    <FieldLabel htmlFor="author">Author</FieldLabel>
                                    <Input id="author" type="text" placeholder="" value={fieldData.author} disabled />
                                </Field>
                                <Field>
                                    <FieldLabel htmlFor="ai">E-Mail</FieldLabel>
                                    <Input id="mail" type="text" placeholder="" value={fieldData.email} disabled />
                                </Field>
                                <Field>
                                    <FieldLabel htmlFor="i">Organisation ID</FieldLabel>
                                    <Input id="oid" type="text" placeholder="" value={fieldData.oid} disabled />
                                </Field>
                                <Field>
                                    <FieldLabel htmlFor="es">Description</FieldLabel>
                                    <Textarea id="desc" placeholder="" value={fieldData.description} disabled />
                                </Field>
                                <Field>
                                    <FieldLabel htmlFor="link">Link</FieldLabel>
                                    <Input id="link" type="text" value={fieldData.link} disabled />
                                </Field>
                                <Field>
                                    <FieldLabel htmlFor="lisc">Liscence</FieldLabel>
                                    <Input id="lisc" type="text" placeholder="" value={fieldData.license} disabled />
                                </Field>
                            </FieldGroup>
                        </CardDescription>
                    </CardHeader>
                </Card>
            </div >
        </PageLayout >
    )
}
