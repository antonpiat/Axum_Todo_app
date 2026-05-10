"use client"

import {useRouter} from "next/navigation";
import {ChangeEvent, SubmitEvent, useState} from "react";

interface AuthFormProps {
    handleAuth: (username: string, password: string) => Promise<any>,
}

export function AuthForm({handleAuth}: AuthFormProps) {
    const router = useRouter();
    const [credentials, setCredentials] = useState({
        username: "",
        password: "",
    });

    function handleChange(event: ChangeEvent<HTMLInputElement>) {
        const {value, name}: { value: string, name: string } = event.currentTarget;
        setCredentials(prevState => {
            return {...prevState, [name]: value}
        });
    }

    function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
        event.preventDefault();
        handleAuth(credentials.username, credentials.password)
            .then((data: any) => {
                console.log(data);
                router.push("/");
            })
            .catch((error: unknown) => {
                console.log(error)
            })
    }

    return (
        <div>
            <form className="mx-auto flex flex-col justify-center items-center" onSubmit={handleSubmit}>
                <div className="mb-5">
                    <label htmlFor="username" className="block mb-2 text-lg font-medium text-gray-900">Username</label>
                    <input type="text" id="username" name="username" onChange={handleChange}
                           value={credentials.username}
                           className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg
                           focus:ring-blue-500 focus:border-blue-500 block min-w-lg p-2.5" placeholder="John Doe"
                           required/>
                </div>
                <div className="mb-5">
                    <label htmlFor="password" className="block mb-2 text-lg font-medium text-gray-900">Password</label>
                    <input type="password" id="password" name="password" onChange={handleChange}
                           value={credentials.password}
                           className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg
                           focus:ring-blue-500 focus:border-blue-500 block min-w-lg p-2.5" required/>
                </div>
                <button type="submit" className="text-white bg-purple-700 hover:bg-purple-800
                focus:outline-none focus:ring-4 focus:ring-purple-300 font-medium rounded-full text-sm px-5 py-2.5 text-center
                mb-2 dark:bg-purple-600 dark:hover:bg-purple-700 dark:focus:ring-purple-900 min-w-sm ">
                    Submit
                </button>
            </form>
        </div>
    )
}