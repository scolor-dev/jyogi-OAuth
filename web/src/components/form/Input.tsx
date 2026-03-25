import type { InputHTMLAttributes } from 'react'

type InputProps = InputHTMLAttributes<HTMLInputElement>

export default function Input({ className, ...props }: InputProps) {
  return (
    <input
      className={[
        'w-full border rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500',
        className,
      ]
        .filter(Boolean)
        .join(' ')}
      {...props}
    />
  )
}
