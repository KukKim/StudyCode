import React from 'react';
import './index.css';
import { HomeButton } from '../button'

export default function LeftMenu() {
  return (
      <div className="leftMenuConatainer">
          <div
            className="leftMenuTopContainer"
          >
            <HomeButton/>
          </div>
          <div
            className="leftMenuBodyContainer"
          >
            <a>
                1번 메뉴
            </a>
            <a>
                2번 메뉴
            </a>
            <a>
                3번 메뉴
            </a>
            <a>
                4번 메뉴
            </a>
            <a>
                5번 메뉴
            </a>
          </div>
      </div>
  );
}

