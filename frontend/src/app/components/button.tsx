import React, { useState } from 'react'
import { AiOutlineLoading3Quarters } from "react-icons/ai";


interface PropsType{
    val:string,
    onclick:()=>void
    extraClass:string,
    isWaiting:boolean
}

// Button Component
export default function  Button(props:PropsType) {
  
  const [isWaiting,setIsWaiting] = useState<boolean>(false);
  
  return (
    <button onClick={()=>{props.onclick()}} className={'flex bg-slate-900 text-slate-100 font-bold uppercase shadow-md py-3 px-3 rounded-md justify-center disabled:cursor-wait ' + props.extraClass} disabled={props.isWaiting}>{props.val} { props.isWaiting && <AiOutlineLoading3Quarters className="animate-spin size-4 self-center mx-2" /> } </button>
  )
}
