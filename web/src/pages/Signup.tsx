import { Link, useNavigate } from 'react-router-dom'
import FormField from '../components/form/FormField'
import Input from '../components/form/Input'
import PasswordInput from '../components/form/PasswordInput'
import SubmitButton from '../components/form/SubmitButton'

export default function Signup() {
  const navigate = useNavigate()

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="bg-white p-8 rounded-lg shadow w-full max-w-sm">
        <h1 className="text-2xl font-bold mb-6 text-center">新規登録</h1>

        <form onSubmit={(e) => { e.preventDefault(); navigate('/') }} className="flex flex-col gap-4">
          <FormField label="ユーザー名">
            <Input type="text" placeholder="username" />
          </FormField>

          <FormField label="パスワード">
            <PasswordInput />
          </FormField>

          <SubmitButton>登録</SubmitButton>
        </form>

        <p className="mt-4 text-sm text-center text-gray-600">
          すでにアカウントをお持ちの方は{' '}
          <Link to="/login" className="text-blue-600 hover:underline">
            ログイン
          </Link>
        </p>
      </div>
    </div>
  )
}
