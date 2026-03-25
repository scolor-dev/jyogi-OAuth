import type { ComponentProps } from 'react'
import Button from './Button'

type SubmitButtonProps = Omit<ComponentProps<typeof Button>, 'type'>

export default function SubmitButton(props: SubmitButtonProps) {
  return <Button type="submit" {...props} />
}
