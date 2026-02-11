import { useNavigate, Link } from "react-router-dom"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldError, FieldLabel } from "@/components/ui/field"
import { useState } from "react"
import { useAuth } from "@/auth/auth"

export default function Login() {
    const navigate = useNavigate()
    const { login } = useAuth()

    const [isSubmitting, setIsSubmitting] = useState(false)
    const [formData, setFormData] = useState({
        username: '',
        password: ''
    })
    const [errors, setErrors] = useState<{
        username?: string
        password?: string
    }>({})

    const handleSubmit = async (data: typeof formData) => {
        try {
            setErrors({})
            if (!data.username) {
                setErrors({ username: "* Username is required" })
                return
            }
            if (!data.password) {
                setErrors({ password: "* Password is required" })
                return
            }
            setIsSubmitting(true)
            await login(data.username, data.password)
            navigate('/')
        } catch (error) {
            setErrors({ password: "* Invalid username or password" })
        } finally {
            setIsSubmitting(false)
        }
    }
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { id, value } = e.target
        setFormData((prev) => ({ ...prev, [id]: value }))
    }
    return (
        <div className="min-h-screen flex items-center justify-center">
            <Card className="w-160 max-w-sm">
                <CardHeader>
                    <CardTitle className="text-2xl">Login</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                    <Field>
                        <FieldLabel htmlFor="username">Username</FieldLabel>
                        <Input id="username" type="text" placeholder="Username" value={formData.username} onChange={handleChange} className={errors.username ? 'border-red-500' : ''} />
                        <FieldError>{errors.username}</FieldError>
                    </Field>
                    <Field>
                        <FieldLabel htmlFor="password">Password</FieldLabel>
                        <Input id="password" type="password" placeholder="Password" value={formData.password} onChange={handleChange} className={errors.password ? 'border-red-500' : ''} />
                        <FieldError>{errors.password}</FieldError>
                    </Field>
                    <Button
                        className="w-full"
                        variant="outline"
                        onClick={() => handleSubmit(formData)}
                        disabled={isSubmitting}>
                        {isSubmitting ? 'Logging in...' : 'Login'}
                    </Button>
                    <p className="text-sm text-center text-gray-400">
                        No account?{" "}
                        <Link to="/register" className="text-blue-400 underline hover:text-blue-500">
                            Register
                        </Link>
                    </p>
                </CardContent>
            </Card>
        </div>
    )
}
