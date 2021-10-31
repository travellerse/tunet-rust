#pragma once

#include <AboutPage.hpp>
#include <ChartPage.hpp>
#include <DetailPage.hpp>
#include <InfoPage.hpp>
#include <Model.hpp>
#include <QMainWindow>
#include <QTabWidget>

struct MainWnd : QMainWindow
{
public:
    MainWnd();
    ~MainWnd() override;

private:
    Model m_model{ this };

    QTabWidget m_root_tab{ this };
    InfoPage m_info_page{ &m_root_tab, &m_model };
    ChartPage m_chart_page{ &m_root_tab, &m_model };
    DetailPage m_detail_page{ &m_root_tab, &m_model };
    AboutPage m_about_page{ &m_root_tab };
};
