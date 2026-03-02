import { useNavigate, Link } from "react-router-dom"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldError, FieldLabel } from "@/components/ui/field"
import { useState } from "react"
import { useAuth } from "@/auth/auth"

export default function Register() {
    const navigate = useNavigate()
    const { register } = useAuth()

    const [isSubmitting, setIsSubmitting] = useState(false)
    const [formData, setFormData] = useState({
        username: 'aze',
        email: 'aze@example.com',
        password: 'aze',
        confirmPassword: 'aze',
    })
    const [errors, setErrors] = useState<{
        username?: string
        email?: string
        password?: string
    }>({})

    const handleSubmit = async (data: typeof formData) => {
        try {
            setErrors({})
            if (!data.username) {
                setErrors({ username: "* Username is required" })
                return
            }
            if (!data.email) {
                setErrors({ email: "* Email is required" })
                return
            }
            if (!data.email.toLowerCase().match(/^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|.(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/)) {
                setErrors({ email: "* Email is invalid" })
                return
            }
            if (data.password !== data.confirmPassword) {
                setErrors({ password: "* Passwords do not match" })
                return
            }
            const PASSWORD_MIN_LENGTH = 3
            if (data.password.length < PASSWORD_MIN_LENGTH) {
                setErrors({ password: `* Password must be at least ${PASSWORD_MIN_LENGTH} characters long` })
                return
            }
            setIsSubmitting(true)
            await register(data.email, data.password, data.username, setErrors)
            navigate('/')
        } catch (error) {
            console.error("Registration failed:", error)
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
                    <CardTitle className="text-2xl">Register</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                    <Field>
                        <FieldLabel htmlFor="username">Username</FieldLabel>
                        <Input id="username" type="text" placeholder="Username" value={formData.username} onChange={handleChange} aria-invalid={!!errors.username} />
                        <FieldError>{errors.username}</FieldError>
                    </Field>
                    <Field>
                        <FieldLabel htmlFor="email">Email</FieldLabel>
                        <Input id="email" type="email" placeholder="name@example.com" value={formData.email} onChange={handleChange} aria-invalid={!!errors.email} />
                        <FieldError>{errors.email}</FieldError>
                    </Field>
                    <Field>
                        <FieldLabel htmlFor="password">Password</FieldLabel>
                        <Input id="password" type="password" placeholder="Password" value={formData.password} onChange={handleChange} aria-invalid={!!errors.password} />
                        <FieldError>{errors.password}</FieldError>
                    </Field>
                    <Field>
                        <FieldLabel htmlFor="confirmPassword">Confirm Password</FieldLabel>
                        <Input id="confirmPassword" type="password" placeholder="Confirm Password" value={formData.confirmPassword} onChange={handleChange} aria-invalid={!!errors.password} />
                        <FieldError>{errors.password}</FieldError>
                    </Field>
                    <Button
                        className="w-full"
                        variant="outline"
                        onClick={() => handleSubmit(formData)}
                        disabled={isSubmitting}>
                        {isSubmitting ? 'Registering...' : 'Register'}
                    </Button>
                    <p className="text-sm text-center text-gray-400">
                        Already an account?{" "}
                        <Link to="/login" className="text-blue-400 underline hover:text-blue-500">
                            Login
                        </Link>
                    </p>
                    <p className="text-sm text-center text-gray-400">
                        <Link to="/" className="text-blue-400 underline hover:text-blue-500">
                            Continue without an account
                        </Link>
                    </p>
                </CardContent>
            </Card>
        </div >
    )
}
