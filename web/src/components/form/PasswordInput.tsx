import type { InputHTMLAttributes } from 'react'
import Input from './Input'

type PasswordInputProps = Omit<InputHTMLAttributes<HTMLInputElement>, 'type'>

export default function PasswordInput(props: PasswordInputProps) {
  return <Input type="password" placeholder="••••••••" {...props} />
}
