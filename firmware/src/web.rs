// SPDX-FileCopyrightText: 2025 Funkeleinhorn <git@funkeleinhorn.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::sync::atomic::Ordering;

use embassy_net::Stack;
use embassy_time::Duration;
use picoserve::{
    response::File,
    routing, AppBuilder, AppRouter, Router,
};

pub struct Application;

impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        picoserve::Router::new()
            .route(
                "/",
                routing::get_service(File::html(include_str!("index.html"))),
            )
            .route(
                "/pause",
                routing::post(|| async {
                    crate::gpio::GPIO_PAUSE_STATE.store(true, Ordering::Relaxed);
                    picoserve::response::Redirect::to("/")
                }),
            )
            .route(
                "/rec",
                routing::post(|| async {
                    crate::gpio::GPIO_REC_STATE.store(true, Ordering::Relaxed);
                    picoserve::response::Redirect::to("/")
                }),
            )
            .route(
                "/stop",
                routing::post(|| async {
                    crate::gpio::GPIO_STOP_STATE.store(true, Ordering::Relaxed);
                    picoserve::response::Redirect::to("/")
                }),
            )
            .route(
                "/rwd",
                routing::post(|| async {
                    crate::gpio::GPIO_RWD_STATE.store(true, Ordering::Relaxed);
                    picoserve::response::Redirect::to("/")
                }),
            )
            .route(
                "/rev",
                routing::post(|| async {
                    crate::gpio::GPIO_REV_STATE.store(true, Ordering::Relaxed);
                    picoserve::response::Redirect::to("/")
                }),
            )
            .route(
                "/fwd",
                routing::post(|| async {
                    crate::gpio::GPIO_FWD_STATE.store(true, Ordering::Relaxed);
                    picoserve::response::Redirect::to("/")
                }),
            )
            .route(
                "/ff",
                routing::post(|| async {
                    crate::gpio::GPIO_FF_STATE.store(true, Ordering::Relaxed);
                    picoserve::response::Redirect::to("/")
                }),
            )
    }
}

pub const WEB_TASK_POOL_SIZE: usize = 2;

pub struct WebApp {
    pub router: &'static Router<<Application as AppBuilder>::PathRouter>,
    pub config: &'static picoserve::Config<Duration>,
}

impl Default for WebApp {
    fn default() -> Self {
        let router = picoserve::make_static!(AppRouter<Application>, Application.build_app());

        let config = picoserve::make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
            })
        );

        Self { router, config }
    }
}

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
pub async fn web_task(
    id: usize,
    stack: Stack<'static>,
    router: &'static AppRouter<Application>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        id,
        router,
        config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}
