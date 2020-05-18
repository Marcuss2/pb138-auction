import React from "react";
import { Item } from "./Item";
import AuctionItem from "../entitites/AuctionItem";
import { Col, Row } from "antd";
import useDataApi from 'use-data-api';
import { getMockAuctions } from "../mocks/mocks";

export const Auctions = () => {
    const [{data, isLoading, isError}, doFetch] = useDataApi(
        'http://localhost:8080/auctions/',
        [],
    );
    // const auctions = data
    const auctions = getMockAuctions();

    return (
        <Row gutter={[16, 16]}>
            {auctions.map((auction: AuctionItem) => {
                return (
                    <Col xs={24} sm={12} md={8} lg={6} xxl={4}>
                        <Item item={auction} key={auction.id}/>
                    </Col>
                )
            })}
        </Row>
    )
};