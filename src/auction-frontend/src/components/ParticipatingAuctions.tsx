import { observer } from "mobx-react-lite";
import React, { useState } from "react";
import AuctionItem from "../entitites/AuctionItem";
import { Col, Divider, Row } from "antd";
import { Item } from "./Item";
import { getMockAuctions } from "../mocks/mocks";

export const ParticipatingAuctions = observer(() => {

    const [auctions, setAuctions] = useState(getMockAuctions);

    return (
        <div>
            <Divider><h2>Participating auctions</h2></Divider>
            <Row gutter={[16, 16]}>
                {auctions.map((auction: AuctionItem) => {
                    return (
                        <Col xs={24}>
                            <Item item={auction} key={auction.id}/>
                        </Col>
                    )
                })}
            </Row>
        </div>
    )
});