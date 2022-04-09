use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 30000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/", get(app_endpoint))
                .into_make_service(),
        )
        .await
        .unwrap();
}

async fn app_endpoint() -> Html<String> {
    Html(dioxus::ssr::render_lazy(rsx! {
        link { href:"https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel:"stylesheet" }
        div {
            header { class: "text-gray-400 bg-gray-900 body-font",
                div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                    a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                        href: "https://github.com/livstyle",
                        target: "_blank",
                        span { class: "ml-3 text-xl", "LivStyle"}
                    }
                    nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                        a { class: "mr-5 hover:text-white", "最新活动"}
                        a { class: "mr-5 hover:text-white", "产品服务"}
                        a { class: "mr-5 hover:text-white", "解决方案"}
                        a { class: "mr-5 hover:text-white", "关于我们"}
                    }
                    button {
                        class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                        "登录/注册"
                    }
                }
            }

            section { class: "text-gray-400 bg-gray-900 body-font",
                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
                        h1 { class: "title-font sm:text-4xl text-3xl mb-4 font-medium text-white",
                            br { class: "hidden lg:inline-block" }
                            "Rust 学习网站"
                        }
                        p {
                            class: "mb-8 leading-relaxed",

                            "本学习网站主要是涵盖但不限于Rust的学习与分享"

                            // "Dioxus is a new UI framework that makes it easy and simple to write cross-platform apps using web
                            // technologies! It is functional, fast, and portable. Dioxus can run on the web, on the desktop, and
                            // on mobile and embedded platforms."

                        }
                        div { class: "flex justify-center",
                            button {
                                class: "inline-flex text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                                "学习更多"
                            }
                            button {
                                class: "ml-4 inline-flex text-gray-400 bg-gray-800 border-0 py-2 px-6 focus:outline-none hover:bg-gray-700 hover:text-white rounded text-lg",
                                "实践"
                            }
                        }
                    }
                    div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
                        img {
                            class: "object-cover object-center rounded",
                            src: "https://gimg2.baidu.com/image_search/src=http%3A%2F%2Fpic1.zhimg.com%2Fv2-839798432500b3aec901cba0efb93bf7_1440w.jpg%3Fsource%3D172ae18b&refer=http%3A%2F%2Fpic1.zhimg.com&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=auto?sec=1652110101&t=d6448f91625d0ce45894d8ce888d2967",
                            referrerpolicy:"no-referrer",
                            alt: "hero",
                        }
                    }
                }
            }

            div { class: "flex flex-row justify-center",
                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
                        img {
                            class: "object-cover object-center rounded-xl",
                            src: "https://gimg2.baidu.com/image_search/src=http%3A%2F%2Fimg-blog.csdnimg.cn%2Fimg_convert%2F5f75ea9d3201af104c4e517f75710404.png&refer=http%3A%2F%2Fimg-blog.csdnimg.cn&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=auto?sec=1652113216&t=13fd1badc90d6669a4a878ddd18a8cf8",
                            referrerpolicy:"no-referrer",
                            alt: "hero",
                        }
                    }            
                }

                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
                        img {
                            class: "object-cover object-center rounded",
                            src: "https://img2.baidu.com/it/u=2834938743,3407068300&fm=253&fmt=auto&app=138&f=PNG?w=850&h=425",
                            referrerpolicy:"no-referrer",
                            alt: "hero",
                        }
                    }            
                }

                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
                        img {
                            class: "object-cover object-center rounded-xl",
                            src: "https://img2.baidu.com/it/u=3480281359,2103984758&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=281",
                            referrerpolicy:"no-referrer",
                            alt: "hero",
                        }
                    }            
                }                

            }

        }
    }))
}
