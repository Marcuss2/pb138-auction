import AuctionItem from "../entitites/AuctionItem";
import { useDataApi } from "../api/useDataApi";
import { useContext } from "react";
import { UserContext } from "../App";

export const useSortedAuctions = (url: string, initialData: AuctionItem[]) => {
    const [{data, isLoading, isError}, doFetch] = useDataApi(
        url,
        initialData,
    );

    data.sort((a: AuctionItem, b: AuctionItem) =>
        (a.until < b.until) ? 1 : -1
    )

    // Adds to each AuctionItem loosing flag
    const userContext = useContext(UserContext);

    const [{data: participatingAuctions}] = useDataApi(
        `http://localhost:8080/api/auctions-taken-part/user/${userContext.userState.user.id}`,
        [],
    );

    data.forEach((auction: AuctionItem) => auction.loosing = participatingAuctions.some((a: AuctionItem) => a.id === auction.id) && auction.winningUser !== userContext.userState.user.name);

    return [{data, isLoading, isError}, doFetch];
}