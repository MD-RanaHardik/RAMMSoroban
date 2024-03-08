import Image from 'next/image'
import React from 'react'

export default function MaintenanceMode() {
  return (
    <div className='flex items-center justify-center align-middle h-screen'>
        <div className='self-center text-center'>
            <Image className='h-1/2 w-1/2 mx-auto' src={"https://adravity.com/assets/images/MAINTENANCE.png"} width={0} height={0} sizes='100vh' alt='not found'/>
            <h1 className=' text-3xl mt-5 font-extrabold'>We&apos;re down for maintenance</h1>
        </div>
    </div>
  )
}
