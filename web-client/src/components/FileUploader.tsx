import { Button } from "@/components/ui/button"
import { useState } from "react";
import { Card, CardDescription, CardFooter, CardHeader, CardTitle } from "./ui/card";
import { Field, FieldError, FieldGroup, FieldLabel } from "./ui/field";
import { Input } from "./ui/input";
import { Textarea } from "./ui/textarea";
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from "./ui/select";
import { Spinner } from "./ui/spinner";
import FilePicker from "./FilePicker";

export default function FileUploader() {
    const [file, setFile] = useState<File | null>();
    const [loading, setLoading] = useState<boolean>(false)
    const [formData, setFormData] = useState({
        name: '',
        extension: '',
        desc: '',
        lisc: '',
        link: ''
    })
    const [errors, setErrors] = useState<{
        name?: string
        desc?: string
        extension?: string
    }>({})

    const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8888';


    const handleFileDrop = (files: File[]) => {
        if (files.length != 1)
            throw new Error('Single file required')
        let file = files[0]
        setFile(file)
        let filename = file.name
        let extension = ""
        if (filename.includes(".")) {
            filename = filename.substring(0, filename.lastIndexOf("."));
            extension = file.name.substring(file.name.lastIndexOf("."));
        }

        setFormData((prev) => ({ ...prev, name: filename, extension: extension }))
    };

    const handleChange = (e: React.ChangeEvent<HTMLInputElement> | React.ChangeEvent<HTMLTextAreaElement>) => {
        const { id, value } = e.target
        setFormData((prev) => ({ ...prev, [id]: value }))
    }

    const uploadFile = async () => {
        const token = localStorage.getItem("token");

        if (!file) return;
        if (!formData.name) {
            setErrors({ name: "* File name is required" })
            return
        }
        if (!formData.extension) {
            setErrors({ extension: "* File extension is required" })
            return
        }
        try {
            setLoading(true)
            const data = new FormData();

            const jsonBlob = new Blob(
                [JSON.stringify({
                    name: `${formData.name}${formData.extension}`,
                    description: formData.desc,
                    link: formData.link,
                    licence: formData.lisc
                })],
                { type: "application/json" }
            );

            data.append("json", jsonBlob);
            data.append("file", file);

            const response = await fetch(`${API_URL}/file`, {
                method: "POST",
                headers: {
                    Authorization: `Bearer ${token}`,
                },
                body: data,
            });

            if (!response.ok) {
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

    return (
        <Card className="w-4xl flex-4">
            <CardHeader>
                <CardTitle>Uploader you file here</CardTitle>
            </CardHeader>
            <CardDescription className="flex gap-6 flex-col px-6 [&>div]:w-full">
                <FilePicker handleCallBack={handleFileDrop} file={file} setFile={setFile} />
            </CardDescription>

            {(file) && <>
                <CardDescription className="flex gap-6 flex-col px-6 [&>div]:w-full">
                    <FieldGroup className="mt-6">
                        <Field>
                            <FieldLabel htmlFor="name">Name</FieldLabel>
                            <div className="flex gap-2">
                                <Input className="flex-4" id="name" type="text" placeholder="Name" value={formData.name} onChange={handleChange} aria-invalid={!!errors.name} />
                                <Input className="flex-1" id="extension" type="text" placeholder="Extension" value={formData.extension} aria-invalid={!!errors.extension} disabled />
                            </div>
                            <FieldError>{errors.name}</FieldError>
                        </Field>
                        <Field>
                            <FieldLabel htmlFor="desc">Description</FieldLabel>
                            <Textarea id="desc" placeholder="Description" value={formData.desc} onChange={handleChange} aria-invalid={!!errors.desc} />
                            <FieldError>{errors.desc}</FieldError>
                        </Field>
                        <Field>
                            <FieldLabel htmlFor="link">Link</FieldLabel>
                            <Input id="link" type="text" placeholder="Link" value={formData.link} onChange={handleChange} />
                        </Field>
                        <Field>
                            <FieldLabel htmlFor="lisc">Liscence</FieldLabel>
                            <Select defaultValue="mp4" onValueChange={(value) => setFormData((prev) => ({ ...prev, lisc: value }))}>
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder="Select a liscence" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectItem value="mit">MIT</SelectItem>
                                    </SelectGroup>
                                    <SelectGroup>
                                        <SelectItem value="gpl3">GPL v3</SelectItem>
                                    </SelectGroup>
                                    <SelectGroup>
                                        <SelectItem value="cc0">CC0</SelectItem>
                                    </SelectGroup>
                                    <SelectGroup>
                                        <SelectItem value="bsd3">BSD 3</SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </Field>
                    </FieldGroup>
                </CardDescription>
                <CardFooter className="flex gap-6">
                    <Button className="flex-1" variant="outline" id="clear-btn" onClick={() => { setFile(null) }}>Clear</Button>
                    <Button className="flex-1" onClick={() => { uploadFile() }}>{loading && <Spinner />}Upload</Button>
                </CardFooter>
            </>}
        </Card >
    )
}
