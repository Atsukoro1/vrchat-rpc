// TODO: Infinite loading, solve later when needed

// import { invoke } from "@tauri-apps/api"
// import { InvokeArgs } from "@tauri-apps/api/tauri";

// export const invokeTauriFn = async <T,>(
//     fnName: string,
//     args: InvokeArgs | undefined
// ): Promise<T> => {
//     const authCookie = localStorage.getItem("authCookie");

//     return await invoke(fnName, {
//         args,
//         ...(authCookie && { authCookie })
//     });
// }