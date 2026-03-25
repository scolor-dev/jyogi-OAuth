import type { ButtonHTMLAttributes } from 'react'

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement>

export default function Button({ className, children, ...props }: ButtonProps) {
  return (
    <button
      className={[
        'w-full rounded-md py-2 font-medium transition-colors bg-blue-600 text-white hover:bg-blue-700',
        className,
      ]
        .filter(Boolean)
        .join(' ')}
      {...props}
    >
      {children}
    </button>
  )
}
