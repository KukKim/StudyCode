import React from 'react';
import './index.css';
import { HomeButton } from '../button'

export default function LeftMenu() {
  return (
      <div className="leftMenuConatainer">
          <HomeButton/>
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
  );
}

